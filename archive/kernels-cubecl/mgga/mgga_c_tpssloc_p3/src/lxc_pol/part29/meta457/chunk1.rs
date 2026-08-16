//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1777/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1777<F: Float>(t23083: F, t6606: F, t2679: F, t815: F, t6605: F, t2684: F, t23043: F, t23044: F, t23049: F, t23051: F, t23054: F, t23057: F, t23059: F, t23063: F, t23067: F, t23071: F, t23073: F, t23081: F) -> (F, F, F, F) {
    let t23084 = t23083 * t6606;
    let t23086 = t815 * t2679;
    let t23087 = t6605 * t23086;
    let t23089 = t815 * t2684;
    let t23090 = t6605 * t23089;
    let t23092 = t23043 - t23044 / F::cast_from(1536.0_f64) + t23049 / F::cast_from(768.0_f64) - t23051 / F::cast_from(1536.0_f64) - t23054 / F::cast_from(768.0_f64) + t23057 / F::cast_from(16.0_f64) - t23059 / F::cast_from(48.0_f64) + F::cast_from(0.16956557559538964159e-1_f64) * t23063 - F::cast_from(0.12111826828242117256e-2_f64) * t23067 + t23071 + F::cast_from(0.40372756094140390854e-3_f64) * t23073 + F::cast_from(0.84782787797694820792e-2_f64) * t23081 + F::cast_from(0.28260929265898273598e-2_f64) * t23084 - F::cast_from(0.20186378047070195427e-3_f64) * t23087 - F::cast_from(0.20186378047070195427e-3_f64) * t23090;
    (t23084, t23086, t23089, t23092)
}
