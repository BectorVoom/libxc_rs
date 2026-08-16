//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1407/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1407(t21998: f64, t22325: f64, t22344: f64, t22384: f64, t1427: f64, t213: f64, t6888: f64, t13727: f64, t13733: f64, t13737: f64, t1424: f64, t1445: f64, t4071: f64, t5715: f64, t5775: f64, t6896: f64, t9632: f64, t9639: f64, t9642: f64, t9650: f64, t9666: f64) -> f64 {
    let t22386 = t21998 + t22325 + t22344 + t22384;
    let t22387 = t1427 * t22386;
    let t22390 = t213 * t6888;
    let t22393 = 0.73171657588172351096e-2_f64 * t9632 - 0.13170898365871023197e1_f64 * t5715 * t5775 + t9639 - 0.65049603595885220126e-3_f64 * t9642 + t9650 - 0.13009920719177044025e-2_f64 * t13727 - t13733 - t13737 + 0.13170898365871023197e1_f64 * t4071 * t6896 - 0.65854491829355115987e0_f64 * t1424 * t22387 - t9666 - 0.65854491829355115987e0_f64 * t22390 * t1445;
    t22393
}
