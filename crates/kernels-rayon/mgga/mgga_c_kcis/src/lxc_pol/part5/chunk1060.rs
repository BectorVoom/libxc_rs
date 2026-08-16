//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1060/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1060(t1962: f64, t4016: f64, t1014: f64, t5872: f64, t1928: f64, t4161: f64, t2820: f64, t5659: f64, t86: f64, t5664: f64, t11913: f64, t5656: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17250 = t4016 * t1962;
    let t17259 = t1014 * t5872;
    let t17260 = 0.33163888888888888888e-2_f64 * t17259;
    let t17261 = t4161 * t1928;
    let t17266 = t86 * t2820 * t5659;
    let t17267 = t17266 * t5664;
    let t17268 = 0.3684876543209876543e-2_f64 * t17267;
    let t17274 = t11913 * t5656;
    (t17250, t17259, t17260, t17261, t17267, t17268, t17274)
}
