//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3752/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3752<F: Float>(t127: F, t17693: F, t20944: F, t20946: F, t1285: F, t57659: F, t17350: F, t17934: F, t5297: F, t606: F, t1248: F, t12787: F, t12866: F, t13046: F, t1715: F, t17353: F, t17380: F, t17455: F, t17654: F, t17658: F, t17662: F, t17687: F, t17688: F, t17696: F, t1790: F, t21040: F, t3588: F, t3604: F, t3625: F, t372: F, t44550: F, t44951: F, t5056: F, t56997: F, t57663: F, t59078: F, t59362: F, t71200: F, t71314: F, t73: F) -> (F, F, F) {
    let t71435 = t17693 * t127 * t20944 * t20946;
    let t71440 = t1285 * t57659;
    let t71447 = t17934 * t17350;
    let t71452 = t5297 * t606;
    let t71457 = -F::cast_from(0.11433071498151929859e-2_f64) * t17654 * t17353 * t3604 * t5056 * t1248 - F::cast_from(0.57165357490759649296e-3_f64) * t17654 * t17353 * t3604 * t1715 * t3588 - F::cast_from(0.17149607247227894789e-2_f64) * t56997 * t17353 * t13046 * t71200 + F::cast_from(0.23818898954483187207e-3_f64) * t3625 * t12787 * t21040 * t17688 + F::cast_from(0.34299214494455789578e-2_f64) * t44550 * t17350 * t1790 * t17455 + F::cast_from(0.57165357490759649296e-3_f64) * t57663 * t17662 + F::cast_from(0.3811023832717309953e-3_f64) * t59078 + F::cast_from(0.6351706387862183255e-3_f64) * t71435 - F::cast_from(0.1270341277572436651e-2_f64) * t17693 * t59362 * t71314 - F::cast_from(0.5081365110289746604e-2_f64) * t71440 * t17696 - F::cast_from(0.17149607247227894789e-2_f64) * t44951 * t17350 * t1790 * t17380 - F::cast_from(0.11433071498151929859e-2_f64) * t71447 * t17658 - F::cast_from(0.95275595817932748826e-3_f64) * t12866 * t372 * t20944 * t73 * t17687 * t71452;
    (t71440, t71452, t71457)
}
