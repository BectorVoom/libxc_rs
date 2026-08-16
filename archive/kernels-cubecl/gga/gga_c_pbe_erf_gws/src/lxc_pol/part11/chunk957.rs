//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 957/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk957<F: Float>(t2718: F, t8143: F, t1365: F, t974: F, t133: F, t19342: F, t25593: F, t496: F, t2704: F, t2890: F, t8159: F, t5853: F, t981: F) -> (F, F, F, F, F, F, F) {
    let t25619 = t8143 * t2718;
    let t25635 = t1365 * t974;
    let t25636 = t133 * t25635;
    let t25773 = t19342 * t25593;
    let t25828 = t496 * t25635;
    let t25857 = t2890 * t2704;
    let t25866 = t8159 * t2718;
    let t25918 = t981 * t5853;
    (t25619, t25636, t25773, t25828, t25857, t25866, t25918)
}
