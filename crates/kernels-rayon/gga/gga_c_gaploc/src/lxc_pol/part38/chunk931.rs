//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 931/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk931(t45800: f64, t13651: f64, t2197: f64, t1445: f64, t44973: f64, t833: f64, t45087: f64, t13555: f64, t2103: f64, t4673: f64, t13644: f64, t5782: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45801 = 0.14896037479937677779e-1_f64 * t45800;
    let t45803 = 0.11502877786176224903e2_f64 * t2197 * t13651;
    let t45806 = 0.11502877786176224903e2_f64 * t833 * t1445 * t44973;
    let t45809 = 0.11502877786176224903e2_f64 * t833 * t1445 * t45087;
    let t45812 = 0.47667319935800568892e0_f64 * t2103 * t4673 * t13555;
    let t45817 = 0.62115540045351614476e2_f64 * t5782 * t13644;
    (t45801, t45803, t45806, t45809, t45812, t45817)
}
