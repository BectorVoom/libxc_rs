//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1280/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1280(t33094: f64, t10847: f64, t22706: f64, t7584: f64, t16455: f64, t32889: f64, t7585: f64, t10948: f64, t33067: f64, t33069: f64, t33072: f64, t33074: f64, t33077: f64, t33079: f64, t33080: f64, t33081: f64, t33084: f64, t33090: f64, t33092: f64, t7736: f64) -> f64 {
    let t33095 = 0.17041300423964777634e0_f64 * t33094;
    let t33098 = 0.30674340763136599742e2_f64 * t7584 * t22706 * t10847;
    let t33101 = 0.23005755572352449806e2_f64 * t16455 * t7585 * t32889;
    let t33102 = -t33067 + t33069 + t33072 + t33074 - t33077 - t33079 - t33080 + t33081 - t33084 - 0.10725146985555128001e1_f64 * t10948 * t7736 + t33090 + t33092 + t33095 - t33098 + t33101;
    t33102
}
