//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1283/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1283<F: Float>(t270: F, t276: F, t39267: F, t2799: F, t2807: F, t2798: F, t273: F, t2815: F, t10588: F, t896: F, t10595: F, t10599: F) -> (F, F, F, F, F, F, F, F) {
    let t41935 = F::cast_from(1.0_f64) / t276 / t39267 / t270 / F::cast_from(96.0_f64);
    let t41936 = t2799 * t2799;
    let t41937 = t41935 * t41936;
    let t41939 = t2807 * t2807;
    let t41940 = t2798 * t41939;
    let t41942 = F::powf(t273, -F::cast_from(0.25e1_f64));
    let t41943 = t41942 * t41936;
    let t41945 = t2815 * t41939;
    let t41948 = t2798 * t10588 * t896;
    let t41951 = t2815 * t10588 * t896;
    let t41954 = t10595 * t2799 * t2807;
    let t41957 = t10599 * t2799 * t2807;
    (t41937, t41940, t41943, t41945, t41948, t41951, t41954, t41957)
}
