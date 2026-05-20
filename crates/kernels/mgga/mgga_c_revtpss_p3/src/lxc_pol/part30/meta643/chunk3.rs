//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2252/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2252<F: Float>(t13181: F, t7635: F, t1209: F, t7642: F, t26948: F, t29135: F, t5219: F, t7627: F, t105202: F, t1203: F, t1214: F, t1215: F, t1287: F, t1294: F, t1775: F, t17964: F, t1811: F, t2151: F, t2152: F, t26922: F, t26949: F, t26951: F, t26983: F, t27025: F, t29118: F, t29178: F, t29186: F, t29297: F, t29308: F, t3568: F, t3588: F, t5428: F, t72861: F, t7632: F, t7636: F, t7637: F, t7639: F, t7643: F, t7652: F, t8190: F, t8208: F, t97402: F, t97422: F) -> F {
    let t105403 = t7635 * t13181;
    let t105404 = t1209 * t105403;
    let t105409 = t7642 * t105403;
    let t105420 = t26948 * t29135;
    let t105433 = t5219 * t7627;
    let t105442 = t1209 * t105202;
    let t105457 = -F::cast_from(0.10408353825846239354e2_f64) * t105404 * t2151 * t5428 * t1203 + F::cast_from(0.10408353825846239354e2_f64) * t105409 * t2151 * t72861 - F::cast_from(0.8673628188205199462e0_f64) * t26983 * t1811 * t2152 - F::cast_from(0.34694512752820797848e1_f64) * t7643 * t7652 * t29118 * t1294 - F::cast_from(0.26020884564615598386e1_f64) * t105420 * t26951 + F::cast_from(0.34694512752820797848e1_f64) * t7636 * t7652 * t29186 * t1294 - F::cast_from(0.13170898365871023197e1_f64) * t97402 * t1775 + F::cast_from(0.34694512752820797848e1_f64) * t7636 * t7652 * t29178 * t1203 - F::cast_from(0.13170898365871023197e1_f64) * t105433 * t1215 - F::cast_from(0.26020884564615598386e1_f64) * t26949 * t7637 * t8190 * t3568 + F::cast_from(0.34694512752820797848e1_f64) * t27025 * t29297 - F::cast_from(0.17347256376410398924e1_f64) * t105442 * t7639 + F::cast_from(0.17347256376410398924e1_f64) * t97422 * t29308 - F::cast_from(0.34694512752820797848e1_f64) * t7643 * t7652 * t29178 * t1214 + F::cast_from(0.8673628188205199462e0_f64) * t26922 * t8208 * t3588 * t1287 - F::cast_from(0.65854491829355115987e0_f64) * t7632 * t17964;
    t105457
}
