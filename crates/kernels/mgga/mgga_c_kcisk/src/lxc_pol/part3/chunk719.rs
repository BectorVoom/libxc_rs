//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 719/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk719<F: Float>(t11184: F, t5015: F, t10817: F, t10845: F, t10849: F, t10852: F, t10856: F, t10863: F, t10866: F, t10869: F, t10876: F, t10881: F, t10884: F, t10888: F, t11169: F, t11172: F, t11176: F, t11181: F, t1773: F, t1787: F, t4989: F, t5013: F, t5017: F, t5040: F, t664: F) -> F {
    let t11185 = t5015 * t11184;
    let t11188 = F::new(0.32383419689119170984e0) * t1773 * t10845 - F::new(0.53972366148531951639e-1) * t5013 * t10849 - F::new(0.10794473229706390328e0) * t5013 * t10852 - F::new(0.10794473229706390328e0) * t10856 * t5017 - F::new(0.16191709844559585492e0) * t4989 * t5040 - F::new(0.16191709844559585492e0) * t10817 * t1787 - F::new(0.10794473229706390328e0) * t10863 + F::new(0.35981577432354634426e-1) * t10866 - F::new(0.53972366148531951639e-1) * t10869 - F::new(0.32383419689119170984e0) * t1773 * t10876 - F::new(0.11993859144118211475e-1) * t10881 + F::new(0.17990788716177317213e-1) * t10884 + F::new(0.2398771828823642295e-1) * t10888 + F::new(0.5397236614853195164e-1) * t11169 * t664 + F::new(0.71963154864709268853e-1) * t5013 * t11172 - F::new(0.53972366148531951639e-1) * t5013 * t11176 + F::new(0.10794473229706390328e0) * t5013 * t11181 + F::new(0.10794473229706390328e0) * t5013 * t11185;
    t11188
}
