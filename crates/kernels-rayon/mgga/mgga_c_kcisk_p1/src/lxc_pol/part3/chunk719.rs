//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 719/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk719(t11184: f64, t5015: f64, t10817: f64, t10845: f64, t10849: f64, t10852: f64, t10856: f64, t10863: f64, t10866: f64, t10869: f64, t10876: f64, t10881: f64, t10884: f64, t10888: f64, t11169: f64, t11172: f64, t11176: f64, t11181: f64, t1773: f64, t1787: f64, t4989: f64, t5013: f64, t5017: f64, t5040: f64, t664: f64) -> f64 {
    let t11185 = t5015 * t11184;
    let t11188 = 0.32383419689119170984e0_f64 * t1773 * t10845 - 0.53972366148531951639e-1_f64 * t5013 * t10849 - 0.10794473229706390328e0_f64 * t5013 * t10852 - 0.10794473229706390328e0_f64 * t10856 * t5017 - 0.16191709844559585492e0_f64 * t4989 * t5040 - 0.16191709844559585492e0_f64 * t10817 * t1787 - 0.10794473229706390328e0_f64 * t10863 + 0.35981577432354634426e-1_f64 * t10866 - 0.53972366148531951639e-1_f64 * t10869 - 0.32383419689119170984e0_f64 * t1773 * t10876 - 0.11993859144118211475e-1_f64 * t10881 + 0.17990788716177317213e-1_f64 * t10884 + 0.2398771828823642295e-1_f64 * t10888 + 0.5397236614853195164e-1_f64 * t11169 * t664 + 0.71963154864709268853e-1_f64 * t5013 * t11172 - 0.53972366148531951639e-1_f64 * t5013 * t11176 + 0.10794473229706390328e0_f64 * t5013 * t11181 + 0.10794473229706390328e0_f64 * t5013 * t11185;
    t11188
}
