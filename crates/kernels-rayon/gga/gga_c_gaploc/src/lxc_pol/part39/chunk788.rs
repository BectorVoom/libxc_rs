//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 788/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk788(t13800: f64, t1445: f64, t574: f64, t12539: f64, t12941: f64, t12944: f64, t12946: f64, t12948: f64, t12952: f64, t12955: f64, t12959: f64, t13789: f64, t13793: f64, t13795: f64, t13796: f64, t13798: f64) -> (f64, f64) {
    let t13801 = t1445 * t13800;
    let t13802 = t574 * t13801;
    let t13804 = -t12941 - 0.14896037479937677779e-1_f64 * t12944 + 0.14896037479937677779e-1_f64 * t12946 + 0.14896037479937677779e-1_f64 * t13789 - 0.14896037479937677779e-1_f64 * t13793 - t12948 + t13795 - t13796 - 0.76685851907841499354e0_f64 * t12539 + t12952 - t12955 - t12959 + 0.35750489951850426669e0_f64 * t13798 - 0.46011511144704899612e1_f64 * t13802;
    (t13801, t13804)
}
