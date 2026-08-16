//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1146/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1146<F: Float>(t39344: F, t761: F, t39362: F, t2751: F, t39494: F, t153: F, t157: F, t39842: F, t2374: F, t39354: F, t39516: F, t39325: F) -> (F, F, F, F, F, F, F, F) {
    let t40764 = F::cast_from(0.46785788981077169656e1_f64) * t761 * t39344;
    let t40766 = F::cast_from(0.62337092780453269531e3_f64) * t761 * t39362;
    let t40771 = t2751 * t2751;
    let t40772 = F::cast_from(1.0_f64) / t40771;
    let t40779 = F::cast_from(0.51947577317044391277e2_f64) * t761 * t39494;
    let t40784 = t153 * t157 * t39842;
    let t40790 = F::cast_from(0.21687162600603479684e-1_f64) * t2374 * t39354;
    let t40793 = F::cast_from(0.1301229756036208781e0_f64) * t2374 * t39516;
    let t40797 = F::cast_from(0.38025319932552508021e2_f64) * t2374 * t39325;
    (t40764, t40766, t40772, t40779, t40784, t40790, t40793, t40797)
}
