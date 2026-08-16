//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1027/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1027(t25: f64, t265: f64, t394: f64, t123798: f64, t123835: f64, t123428: f64, t123766: f64, t1409: f64, t32072: f64, t34031: f64, t3966: f64, t40: f64, t607: f64, t8760: f64, t116473: f64, t116476: f64, t116481: f64, t119755: f64, t119763: f64, t123378: f64, t123382: f64, t123398: f64, t123414: f64, t123715: f64, t123733: f64, t123752: f64, t1649: f64, t1877: f64, t23788: f64, t24191: f64, t2522: f64, t25892: f64, t25898: f64, t25901: f64, t25905: f64, t25921: f64, t25927: f64, t25930: f64, t25934: f64, t25938: f64, t26756: f64, t28: f64, t32030: f64, t32034: f64, t7109: f64, t7114: f64, t7656: f64, t8748: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t123836 = t123798 + t123835;
    let t123837 = piecewise3(t395, 0.0_f64, t123836);
    let t123844 = piecewise3(t115, t123428 + t123766, t123837 * t40 / 2.0_f64 + t32072 * t1409 / 2.0_f64 + t34031 * t607 / 2.0_f64 + t8760 * t3966 / 2.0_f64);
    let t123888 = t1877 * t123715 * t28 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t116473 * t25898 - t123398 - t1877 * t32034 * t25930 / 2.0_f64 + 3.0_f64 * t123382 * t25892 - 3.0_f64 * t24191 * t23788 * t123414 - 3.0_f64 / 2.0_f64 * t116473 * t25921 + t1877 * t32030 * t1649 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t2522 * t8748 * t25938 + 3.0_f64 * t116481 * t119763 - t1877 * t116476 * t7656 / 2.0_f64 + t123733 - t1877 * t7114 * t1649 * t7109 - t1877 * t32034 * t25934 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t2522 * t8748 * t25901 - 3.0_f64 * t123378 * t119755 + 2.0_f64 * t26756 * t25927 * t123752 - 3.0_f64 / 2.0_f64 * t2522 * t8748 * t25905;
    (t123836, t123844, t123888)
}
