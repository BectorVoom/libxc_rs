//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 946/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk946(t14675: f64, t446: f64, t2876: f64, t4051: f64, t10248: f64, t4057: f64, t681: f64, t89: f64, t14619: f64, t14622: f64, t14626: f64, t14630: f64, t14633: f64, t14636: f64, t14638: f64, t14640: f64, t14642: f64, t14645: f64, t14650: f64, t14655: f64, t14658: f64, t14662: f64, t14666: f64, t14669: f64, t14673: f64) -> (f64, f64, f64, f64, f64) {
    let t14676 = t446 * t14675;
    let t14678 = t4051 * t2876;
    let t14679 = t10248 * t14678;
    let t14680 = t446 * t14679;
    let t14683 = t89 * t681 * t4057;
    let t14684 = 2.0_f64 / 9.0_f64 * t14683;
    let t14685 = -4.0_f64 / 9.0_f64 * t14619 + 4.0_f64 / 27.0_f64 * t14622 + t14626 / 18.0_f64 - 2.0_f64 / 9.0_f64 * t14630 + t14633 / 9.0_f64 - t14636 - t14638 + t14640 - t14642 / 27.0_f64 - 5.0_f64 / 81.0_f64 * t14645 - t14650 / 9.0_f64 + t14655 / 27.0_f64 - t14658 + t14662 / 9.0_f64 + t14666 / 18.0_f64 + 2.0_f64 / 9.0_f64 * t14669 - t14673 / 9.0_f64 - t14676 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t14680 - t14684;
    (t14676, t14678, t14680, t14683, t14685)
}
