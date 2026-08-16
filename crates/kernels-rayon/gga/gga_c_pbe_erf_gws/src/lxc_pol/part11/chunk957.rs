//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 957/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk957(t2718: f64, t8143: f64, t1365: f64, t974: f64, t133: f64, t19342: f64, t25593: f64, t496: f64, t2704: f64, t2890: f64, t8159: f64, t5853: f64, t981: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
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
