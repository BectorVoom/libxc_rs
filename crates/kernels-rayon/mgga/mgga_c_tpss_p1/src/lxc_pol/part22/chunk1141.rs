//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1141/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1141(t12710: f64, t12725: f64, t162: f64, t189: f64, t489: f64, t9841: f64, t3245: f64, t541: f64, t1206: f64, t12673: f64, t12678: f64, t12679: f64, t12688: f64, t12690: f64, t12692: f64, t1625: f64, t198: f64, t3183: f64, t3184: f64, t3387: f64, t4478: f64, t4524: f64, t4525: f64, t4528: f64, t4532: f64, t7929: f64, t7932: f64, t7936: f64, t9839: f64, t9844: f64, t9846: f64, t9848: f64, t9854: f64) -> (f64, f64, f64, f64) {
    let t12727 = (t12710 + t12725) * t162;
    let t12728 = t12727 * t189;
    let t12729 = t489 * t12728;
    let t12730 = 0.21687162600603479684e-1_f64 * t9841;
    let t12731 = t3245 * t541;
    let t12737 = 6.0_f64 * t1206 * t12673 * t3183 - 6.0_f64 * t12679 * t3183 * t4525 + 6.0_f64 * t12731 * t1625 * t198 + 12.0_f64 * t3184 * t4478 * t4532 + 6.0_f64 * t3245 * t4528 * t4532 - t3387 * t4524 * t4525 + t12678 - t12688 - t12690 + t12692 + t12729 + t12730 + t7929 - t7932 - t7936 - t9839 + t9844 + t9846 - t9848 + t9854;
    (t12727, t12729, t12730, t12737)
}
