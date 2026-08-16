//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1651/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1651(t5873: f64, t884: f64, t3071: f64, t10422: f64, t5908: f64, t3070: f64, t1025: f64, t10403: f64, t10923: f64, t10937: f64, t14194: f64, t14203: f64, t14495: f64, t14503: f64, t18008: f64, t18010: f64, t18016: f64, t18021: f64, t18025: f64, t18030: f64, t3117: f64, t378: f64, t5900: f64, t5909: f64) -> (f64, f64, f64, f64, f64) {
    let t18035 = t5873 * t884;
    let t18036 = t3071 * t18035;
    let t18041 = t10422 * t5908;
    let t18042 = t3070 * t18041;
    let t18044 = t18008 / 3456.0_f64 + t14194 - t18010 * t378 / 576.0_f64 - t14203 / 10368.0_f64 + t10403 * t18016 / 1152.0_f64 - t10923 / 1296.0_f64 + t3070 * t18021 / 4608.0_f64 - t3070 * t18025 / 1152.0_f64 + t14495 + t18030 * t1025 / 3072.0_f64 - t3117 * t5900 / 2304.0_f64 + t14503 + t10403 * t18036 / 2304.0_f64 - t10937 * t5909 / 432.0_f64 + t18042 / 3456.0_f64;
    (t18035, t18036, t18041, t18042, t18044)
}
