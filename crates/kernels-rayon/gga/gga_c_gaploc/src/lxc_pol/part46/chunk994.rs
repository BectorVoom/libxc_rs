//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 994/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk994(t1966: f64, t43842: f64, t590: f64, t1890: f64, t43107: f64, t10948: f64, t11016: f64, t13012: f64, t2087: f64, t4614: f64, t1445: f64, t1998: f64, t43306: f64, t43800: f64, t43803: f64, t43806: f64, t43809: f64, t43812: f64, t43815: f64, t43817: f64, t43820: f64, t43822: f64, t43825: f64, t43830: f64, t43833: f64, t43836: f64, t43841: f64, t701: f64) -> f64 {
    let t43844 = t1966 * t43842 * t590;
    let t43849 = 0.25561950635947166451e1_f64 * t1966 * t1890 * t43107 * t590;
    let t43854 = t10948 * t11016;
    let t43858 = 0.92023022289409799224e1_f64 * t2087 * t4614 * t13012;
    let t43859 = t43800 - t43803 + t43806 - t43809 - 0.29792074959875355558e-1_f64 * t43812 + 0.92023022289409799224e1_f64 * t43815 - 0.29792074959875355558e-1_f64 * t43817 + t43820 + t43822 - 0.71500979903700853338e0_f64 * t43825 - t43830 + t43833 + 0.20449560508757733161e1_f64 * t43836 + t43841 - 0.51123901271894332902e1_f64 * t43844 - t43849 - 0.23005755572352449806e1_f64 * t1998 * t1445 * t43306 * t701 - 0.14300195980740170668e1_f64 * t43854 - t43858;
    t43859
}
