//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 667/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk667<F: Float>(t11167: F, t655: F, t10585: F, t7234: F, t1785: F, t4648: F, t5015: F, t5014: F, t5030: F, t1636: F, t5032: F, t4644: F, t10817: F, t10845: F, t10849: F, t10852: F, t10856: F, t10863: F, t10866: F, t10869: F, t10876: F, t10881: F, t10884: F, t10888: F, t1773: F, t1787: F, t4989: F, t5013: F, t5017: F, t5040: F, t664: F, sigma2: F) -> (F,) {
    let t11168 = t11167 * sigma2;
    let t11169 = t11168 * t655;
    let t11172 = t7234 * t10585;
    let t11175 = t4648 * t1785;
    let t11176 = t5015 * t11175;
    let t11179 = t5014 * t5030;
    let t11180 = t1636 * t5032;
    let t11181 = t11179 * t11180;
    let t11184 = t4644 * t1785;
    let t11185 = t5015 * t11184;
    let t11188 = 0.32383419689119170984e0 * t1773 * t10845 - 0.53972366148531951639e-1 * t5013 * t10849 - 0.10794473229706390328e0 * t5013 * t10852 - 0.10794473229706390328e0 * t10856 * t5017 - 0.16191709844559585492e0 * t4989 * t5040 - 0.16191709844559585492e0 * t10817 * t1787 - 0.10794473229706390328e0 * t10863 + 0.35981577432354634426e-1 * t10866 - 0.53972366148531951639e-1 * t10869 - 0.32383419689119170984e0 * t1773 * t10876 - 0.11993859144118211475e-1 * t10881 + 0.17990788716177317213e-1 * t10884 + 0.2398771828823642295e-1 * t10888 + 0.5397236614853195164e-1 * t11169 * t664 + 0.71963154864709268853e-1 * t5013 * t11172 - 0.53972366148531951639e-1 * t5013 * t11176 + 0.10794473229706390328e0 * t5013 * t11181 + 0.10794473229706390328e0 * t5013 * t11185;
    (t11188,)
}
