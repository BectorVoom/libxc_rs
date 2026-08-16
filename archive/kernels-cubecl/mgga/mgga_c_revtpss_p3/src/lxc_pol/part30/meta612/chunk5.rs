//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2100/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2100<F: Float>(t13826: F, t7271: F, t13923: F, t7264: F, t14036: F, t25997: F, t13946: F, t26028: F, t94456: F, t94460: F, t98161: F, t98165: F, t98169: F, t98170: F, t98172: F, t98174: F) -> F {
    let t98176 = t7271 * t13826;
    let t98178 = t7264 * t13923;
    let t98180 = t25997 * t14036;
    let t98181 = F::cast_from(0.50820002809285328226e-4_f64) * t98180;
    let t98182 = t26028 * t13946;
    let t98184 = F::cast_from(0.50820002809285328225e-5_f64) * t98161 - F::cast_from(0.40015750243531754508e-2_f64) * t94456 - F::cast_from(0.22675591804667994222e-1_f64) * t94460 - F::cast_from(0.45351183609335988442e-1_f64) * t98165 - t98169 + t98170 / F::cast_from(8.0_f64) + t98172 / F::cast_from(16.0_f64) + F::cast_from(0.27104001498285508387e-3_f64) * t98174 - F::cast_from(0.51448821741683684367e-1_f64) * t98176 - F::cast_from(0.42874018118069736972e-3_f64) * t98178 - t98181 - F::cast_from(0.85748036236139473944e-3_f64) * t98182;
    t98184
}
