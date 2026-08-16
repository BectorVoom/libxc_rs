//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1260/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1260(t2797: f64, t8024: f64, t2619: f64, t2751: f64, t2753: f64, t1: f64, t11518: f64, t23825: f64, t24978: f64, t25188: f64, t25595: f64, t25776: f64, t26010: f64, t26014: f64, t26016: f64, t26019: f64, t26021: f64, t26023: f64, t2704: f64, t2806: f64, t287: f64, t297: f64, t313: f64, t3907: f64, t7852: f64, t7857: f64, t8002: f64, t8116: f64, t8177: f64, t894: f64, t899: f64, t913: f64, t914: f64, t930: f64, t953: f64) -> f64 {
    let t26027 = t2797 * t8024;
    let t26039 = t2751 * t2619 * t2753;
    let t26049 = -0.17386322979577515709e0_f64 * t930 * t914 * t25188 - 0.10747883617784362088e0_f64 * t2704 * t7852 - 0.49917948358154037253e1_f64 * t26010 * t899 + 0.42929192542166705456e-1_f64 * t26014 - 0.21304854723623629356e5_f64 * t26016 * t8116 + 0.71652557451895747254e-1_f64 * t26019 + 0.1343485452223045261e-1_f64 * t26021 + 0.80609127133382715661e-1_f64 * t26023 + 0.2266661366226402048e1_f64 * t8177 * t2806 - 0.41212024840480037237e0_f64 * t26027 + 0.1343485452223045261e0_f64 * t953 * t894 * t7857 * t23825 + 0.11360101276506094136e1_f64 * t913 * t914 * t287 * t24978 * t297 - 0.30972456242994093474e2_f64 * t26039 + 0.69688026546736710315e2_f64 * t2751 * t313 * t25776 * t1 - 0.18583473745796456084e3_f64 * t3907 * t11518 * t25595 * t8002;
    t26049
}
