//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1154/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1154(t23078: f64, t23080: f64, t6584: f64, t6604: f64, t6606: f64, t2679: f64, t815: f64, t6605: f64, t2684: f64, t23043: f64, t23044: f64, t23049: f64, t23051: f64, t23054: f64, t23057: f64, t23059: f64, t23063: f64, t23067: f64, t23071: f64, t23073: f64) -> (f64, f64, f64, f64) {
    let t23081 = t23078 * t23080;
    let t23083 = t6584 * t6604;
    let t23084 = t23083 * t6606;
    let t23086 = t815 * t2679;
    let t23087 = t6605 * t23086;
    let t23089 = t815 * t2684;
    let t23090 = t6605 * t23089;
    let t23092 = t23043 - t23044 / 1536.0_f64 + t23049 / 768.0_f64 - t23051 / 1536.0_f64 - t23054 / 768.0_f64 + t23057 / 16.0_f64 - t23059 / 48.0_f64 + 0.16956557559538964159e-1_f64 * t23063 - 0.12111826828242117256e-2_f64 * t23067 + t23071 + 0.40372756094140390854e-3_f64 * t23073 + 0.84782787797694820792e-2_f64 * t23081 + 0.28260929265898273598e-2_f64 * t23084 - 0.20186378047070195427e-3_f64 * t23087 - 0.20186378047070195427e-3_f64 * t23090;
    (t23083, t23086, t23089, t23092)
}
