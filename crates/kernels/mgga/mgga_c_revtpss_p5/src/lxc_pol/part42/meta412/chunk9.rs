//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1454/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1454<F: Float>(t21998: F, t22325: F, t22344: F, t22384: F, t1427: F, t213: F, t6888: F, t13727: F, t13733: F, t13737: F, t1424: F, t1445: F, t4071: F, t5715: F, t5775: F, t6896: F, t9632: F, t9639: F, t9642: F, t9650: F, t9666: F) -> F {
    let t22386 = t21998 + t22325 + t22344 + t22384;
    let t22387 = t1427 * t22386;
    let t22390 = t213 * t6888;
    let t22393 = F::cast_from(0.73171657588172351096e-2_f64) * t9632 - F::cast_from(0.13170898365871023197e1_f64) * t5715 * t5775 + t9639 - F::cast_from(0.65049603595885220126e-3_f64) * t9642 + t9650 - F::cast_from(0.13009920719177044025e-2_f64) * t13727 - t13733 - t13737 + F::cast_from(0.13170898365871023197e1_f64) * t4071 * t6896 - F::cast_from(0.65854491829355115987e0_f64) * t1424 * t22387 - t9666 - F::cast_from(0.65854491829355115987e0_f64) * t22390 * t1445;
    t22393
}
