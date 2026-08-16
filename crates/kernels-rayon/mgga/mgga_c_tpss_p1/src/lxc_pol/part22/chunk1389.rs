//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1389/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1389(t20217: f64, t508: f64, t10456: f64, t1163: f64, t1322: f64, t1600: f64, t1760: f64, t1844: f64, t18551: f64, t18627: f64, t18680: f64, t18896: f64, t19577: f64, t20226: f64, t20288: f64, t20361: f64, t20386: f64, t2054: f64, t2056: f64, t20640: f64, t3166: f64, t3245: f64, t3491: f64, t4341: f64, t485: f64, t5706: f64, t5709: f64, t5799: f64, t5895: f64, t5910: f64, t624: f64, t6245: f64, t626: f64, t6309: f64, t63101: f64, t6324: f64, t6399: f64, t67519: f64, t7798: f64) -> f64 {
    let t67782 = t508 * t20217;
    let t67792 = -2.0_f64 * t626 * t1600 * t18627 - 2.0_f64 * t7798 * t6324 - 4.0_f64 * t10456 * t6324 - 4.0_f64 * t2056 * t20386 - t1322 * t18896 - t18680 * t1600 - 2.0_f64 * t5799 * t4341 - t67519 * t485 - 2.0_f64 * t20288 * t1163 - t2054 * t6399 - 2.0_f64 * t624 * t20640 + 3.0_f64 * t1760 * t20226 * t18551 + 6.0_f64 * t19577 * t5910 + 3.0_f64 * t1760 * t63101 * t6245 - 2.0_f64 * t3491 * t5895 - t6309 * t3166 + 6.0_f64 * t1760 * t67782 * t5709 + 6.0_f64 * t1760 * t3245 * t1844 * t6245 - 2.0_f64 * t5706 * t20361;
    t67792
}
