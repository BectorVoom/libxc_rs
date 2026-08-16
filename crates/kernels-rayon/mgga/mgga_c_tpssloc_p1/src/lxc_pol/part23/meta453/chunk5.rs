//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1310/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1310(t39529: f64, t40764: f64, t40766: f64, t40779: f64, t40784: f64, t75894: f64, t75895: f64, t75900: f64, t75901: f64, t75932: f64, t75933: f64, t39549: f64, t39563: f64, t40790: f64, t40793: f64, t40797: f64, t40799: f64, t40801: f64, t40803: f64, t75939: f64, t75940: f64, t75941: f64, t75942: f64) -> (f64, f64) {
    let t76013 = t40764 + t40766 + t75894 + t75895 - t39529 + t75900 - t75901 - t40779 + t75932 + t40784 + t75933;
    let t76014 = t40790 + t40793 + t40797 + t40799 + t40801 - t40803 + t39549 + t75939 + t39563 + t75940 + t75941 + t75942;
    (t76013, t76014)
}
