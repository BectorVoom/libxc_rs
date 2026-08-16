//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1146/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1146(t39344: f64, t761: f64, t39362: f64, t2751: f64, t39494: f64, t153: f64, t157: f64, t39842: f64, t2374: f64, t39354: f64, t39516: f64, t39325: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40764 = 0.46785788981077169656e1_f64 * t761 * t39344;
    let t40766 = 0.62337092780453269531e3_f64 * t761 * t39362;
    let t40771 = t2751 * t2751;
    let t40772 = 1.0_f64 / t40771;
    let t40779 = 0.51947577317044391277e2_f64 * t761 * t39494;
    let t40784 = t153 * t157 * t39842;
    let t40790 = 0.21687162600603479684e-1_f64 * t2374 * t39354;
    let t40793 = 0.1301229756036208781e0_f64 * t2374 * t39516;
    let t40797 = 0.38025319932552508021e2_f64 * t2374 * t39325;
    (t40764, t40766, t40772, t40779, t40784, t40790, t40793, t40797)
}
