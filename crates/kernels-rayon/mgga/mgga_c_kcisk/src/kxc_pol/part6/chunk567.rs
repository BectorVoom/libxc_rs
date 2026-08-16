//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 567/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk567(t1433: f64, t7897: f64, t457: f64, t2110: f64, t1421: f64, t338: f64, t3519: f64, t456: f64, t5893: f64, t5918: f64, t5941: f64, t7828: f64, t7846: f64, t7850: f64, t7854: f64, t7858: f64, t7862: f64, t7866: f64, t7870: f64, t7874: f64, t7879: f64) -> (f64, f64, f64) {
    let t7898 = t1433 * t7897;
    let t7899 = t457 * t7898;
    let t7902 = t2110 * t2110;
    let t7906 = -t3519 + 0.8760572888888888889e-3_f64 * t5893 + 0.19711289e-2_f64 * t5918 - 0.13140859333333333333e-2_f64 * t5941 + 0.10950716111111111111e-2_f64 * t1421 * t7846 + 0.19711289e-2_f64 * t1421 * t7850 - 0.13140859333333333333e-2_f64 * t1421 * t7854 - 0.13140859333333333333e-2_f64 * t1421 * t7858 + 0.65704296666666666667e-3_f64 * t1421 * t7862 + 0.7391733375e-3_f64 * t456 * t7866 - 0.295669335e-2_f64 * t1421 * t7870 + 0.1478346675e-2_f64 * t456 * t7874 + 0.19711289e-2_f64 * t456 * t7879 - 0.98556445e-3_f64 * t456 * t7899 - 4.0_f64 * t7902 - 4.0_f64 * t338 * t7828;
    (t7898, t7899, t7906)
}
