//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3752/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3752(t127: f64, t17693: f64, t20944: f64, t20946: f64, t1285: f64, t57659: f64, t17350: f64, t17934: f64, t5297: f64, t606: f64, t1248: f64, t12787: f64, t12866: f64, t13046: f64, t1715: f64, t17353: f64, t17380: f64, t17455: f64, t17654: f64, t17658: f64, t17662: f64, t17687: f64, t17688: f64, t17696: f64, t1790: f64, t21040: f64, t3588: f64, t3604: f64, t3625: f64, t372: f64, t44550: f64, t44951: f64, t5056: f64, t56997: f64, t57663: f64, t59078: f64, t59362: f64, t71200: f64, t71314: f64, t73: f64) -> (f64, f64, f64) {
    let t71435 = t17693 * t127 * t20944 * t20946;
    let t71440 = t1285 * t57659;
    let t71447 = t17934 * t17350;
    let t71452 = t5297 * t606;
    let t71457 = -0.11433071498151929859e-2_f64 * t17654 * t17353 * t3604 * t5056 * t1248 - 0.57165357490759649296e-3_f64 * t17654 * t17353 * t3604 * t1715 * t3588 - 0.17149607247227894789e-2_f64 * t56997 * t17353 * t13046 * t71200 + 0.23818898954483187207e-3_f64 * t3625 * t12787 * t21040 * t17688 + 0.34299214494455789578e-2_f64 * t44550 * t17350 * t1790 * t17455 + 0.57165357490759649296e-3_f64 * t57663 * t17662 + 0.3811023832717309953e-3_f64 * t59078 + 0.6351706387862183255e-3_f64 * t71435 - 0.1270341277572436651e-2_f64 * t17693 * t59362 * t71314 - 0.5081365110289746604e-2_f64 * t71440 * t17696 - 0.17149607247227894789e-2_f64 * t44951 * t17350 * t1790 * t17380 - 0.11433071498151929859e-2_f64 * t71447 * t17658 - 0.95275595817932748826e-3_f64 * t12866 * t372 * t20944 * t73 * t17687 * t71452;
    (t71440, t71452, t71457)
}
