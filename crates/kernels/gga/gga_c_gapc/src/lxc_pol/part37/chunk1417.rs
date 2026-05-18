//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1417/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1417<F: Float>(t34530: F, t34533: F, t34547: F, t36969: F, t36970: F, t36971: F, t36972: F, t36973: F, t36974: F, t36977: F, t36978: F, t34550: F, t34567: F, t36982: F, t36983: F, t36984: F, t36985: F, t36986: F, t36987: F, t36989: F, t36990: F, t36991: F) -> (F, F) {
    let t38583 = t36969 + t36970 + t36971 + t36972 - t36973 - t36974 + F::new(0.56912804804009946682e-7) * t34530 + F::new(0.68761854623411138864e-8) * t34533 - t36977 + t36978 + F::new(0.56399158975894962978e-8) * t34547;
    let t38586 = F::new(0.90579542097823505428e-7) * t34550 + t36982 + t36983 + t36984 + t36985 - t36986 - t36987 - F::new(0.6629778687778673199e-7) * t34567 + t36989 - t36990 - t36991;
    (t38583, t38586)
}
