//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 884/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk884<F: Float>(t1885: F, t7641: F, t1820: F, t1866: F, t2630: F, t587: F, t1010: F, t5304: F, t1022: F, t1697: F, t1413: F, t1809: F) -> (F, F, F, F) {
    let t7642 = t1885 * t7641;
    let t7644 = F::new(8.0) / F::new(15.0) * t1820 * t7642;
    let t7645 = t2630 * t1866;
    let t7646 = t1885 * t7645;
    let t7648 = F::new(4.0) / F::new(15.0) * t587 * t7646;
    let t7650 = F::new(8.0) / F::new(45.0) * t5304 * t1010;
    let t7651 = t1022 * t1697;
    let t7652 = t7651 * t1413;
    let t7653 = t1809 * t7652;
    (t7644, t7648, t7650, t7653)
}
