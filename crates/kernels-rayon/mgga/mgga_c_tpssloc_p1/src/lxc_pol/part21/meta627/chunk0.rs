//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2408/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2408(t2531: f64, t9722: f64, t39537: f64, t761: f64, t9494: f64, t39344: f64, t39362: f64, t2427: f64, t9868: f64, t2751: f64, t39494: f64, t153: f64, t157: f64, t39842: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40754 = t2531 * t9722;
    let t40760 = 0.12304822629859687989e5_f64 * t761 * t39537;
    let t40761 = t2531 * t9494;
    let t40764 = 0.46785788981077169656e1_f64 * t761 * t39344;
    let t40766 = 0.62337092780453269531e3_f64 * t761 * t39362;
    let t40767 = t2427 * t9868;
    let t40771 = t2751 * t2751;
    let t40772 = 1.0_f64 / t40771;
    let t40779 = 0.51947577317044391277e2_f64 * t761 * t39494;
    let t40784 = t153 * t157 * t39842;
    (t40754, t40760, t40761, t40764, t40766, t40767, t40772, t40779, t40784)
}
