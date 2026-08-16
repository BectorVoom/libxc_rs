//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1338/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1338(t10972: f64, t4614: f64, t813: f64, t29001: f64, t14626: f64, t3483: f64, t10721: f64, t1445: f64, t28988: f64, t28991: f64, t29009: f64, t29011: f64, t29014: f64, t29016: f64, t29019: f64, t29023: f64, t29025: f64, t29032: f64, t29035: f64, t32173: f64, t807: f64) -> f64 {
    let t33891 = 0.12269736305254639897e2_f64 * t813 * t4614 * t10972;
    let t33892 = 0.63904876589867916128e-1_f64 * t29001;
    let t33901 = 0.20449560508757733161e1_f64 * t813 * t14626 * t3483;
    let t33902 = -t33891 - t28988 + t28991 - t33892 + t29009 - t29011 - t29014 + t29016 - t29019 - t29023 + t29025 + t29032 - t29035 + 0.61348681526273199482e1_f64 * t807 * t4614 * t10721 + 0.23005755572352449806e1_f64 * t807 * t1445 * t32173 - t33901;
    t33902
}
