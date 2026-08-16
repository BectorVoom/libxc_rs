//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2546/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2546(t11190: f64, t11191: f64, t1671: f64, t50826: f64, t50919: f64, t43727: f64, t43729: f64, t43748: f64, t43750: f64, t50828: f64, t50832: f64, t50834: f64, t50897: f64, t50900: f64, t50903: f64, t50905: f64, t50907: f64, t50912: f64, t50917: f64, t50921: f64, t50926: f64, t50931: f64, t50934: f64) -> (f64, f64) {
    let t51549 = 24.0_f64 * t11190 * t1671 * t11191;
    let t51550 = 0.23744444444444444444e-1_f64 * t50826;
    let t51565 = 0.15829629629629629629e-1_f64 * t50919;
    let t51570 = t51550 - 0.17808333333333333333e-1_f64 * t50828 + 0.17808333333333333333e-1_f64 * t50832 - 0.18467901234567901234e-1_f64 * t50834 + 0.11872222222222222222e-1_f64 * t43727 - 0.35616666666666666666e-1_f64 * t43729 - 0.15829629629629629629e-1_f64 * t43748 - 0.65956790123456790122e-2_f64 * t43750 - 0.11872222222222222222e-1_f64 * t50897 - 0.42739999999999999999e0_f64 * t50900 - 0.71233333333333333332e-1_f64 * t50903 - 0.35616666666666666666e-1_f64 * t50905 - 0.10685e0_f64 * t50907 + 0.5936111111111111111e-1_f64 * t50912 + 0.23744444444444444444e0_f64 * t50917 - t51565 - 0.19787037037037037036e-1_f64 * t50921 - 0.52765432098765432099e-1_f64 * t50926 + 0.10685e0_f64 * t50931 + 0.10685e0_f64 * t50934;
    (t51549, t51570)
}
