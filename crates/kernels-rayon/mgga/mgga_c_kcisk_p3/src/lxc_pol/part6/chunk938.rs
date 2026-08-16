//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 938/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk938(t29605: f64, t735: f64, t11775: f64, t7320: f64, t9069: f64, t2576: f64, t9086: f64, t2567: f64, t9019: f64, t734: f64, t28963: f64, t747: f64) -> (f64, f64, f64, f64, f64) {
    let t29606 = t735 * t29605;
    let t29607 = t11775 * t29606;
    let t29609 = t7320 * t9069;
    let t29611 = t2576 * t9086;
    let t29613 = t2567 * t9019;
    let t29614 = t734 * t29613;
    let t29616 = t747 * t28963;
    (t29607, t29609, t29611, t29614, t29616)
}
