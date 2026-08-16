//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2171/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2171<F: Float>(t30031: F, t686: F, t72: F, t25878: F, t1955: F, t22307: F, t1903: F, t2030: F, t26084: F, t27960: F, t5774: F, t6896: F, t7295: F, t7296: F, t7910: F, t94735: F, t94756: F, t94758: F, t97951: F, t97953: F, t97956: F, t97964: F, t97968: F, t97974: F) -> (F, F) {
    let t108368 = t30031 * t72 * t686;
    let t108369 = t25878 * t108368;
    let t108371 = t1955 * t22307;
    let t108374 = t97951 + F::cast_from(0.17347256376410398924e1_f64) * t7295 * t7296 * t7910 * t5774 - t97953 + F::cast_from(0.17347256376410398924e1_f64) * t7295 * t7296 * t27960 * t1903 + F::cast_from(0.13170898365871023197e1_f64) * t26084 * t6896 - F::cast_from(0.13009920719177044025e-1_f64) * t94735 + F::cast_from(0.4818682326780666368e-3_f64) * t97956 - F::cast_from(0.96373646535613327357e-2_f64) * t94756 - t97964 + F::cast_from(0.73171657588172351096e-2_f64) * t94758 + F::cast_from(0.25702851531048074406e-1_f64) * t108369 - t97968 + t97974 - F::cast_from(0.4336814094102599731e0_f64) * t108371 * t2030;
    (t108368, t108374)
}
