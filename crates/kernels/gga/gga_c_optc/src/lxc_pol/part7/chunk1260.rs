//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1260/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1260<F: Float>(t2797: F, t8024: F, t2619: F, t2751: F, t2753: F, t1: F, t11518: F, t23825: F, t24978: F, t25188: F, t25595: F, t25776: F, t26010: F, t26014: F, t26016: F, t26019: F, t26021: F, t26023: F, t2704: F, t2806: F, t287: F, t297: F, t313: F, t3907: F, t7852: F, t7857: F, t8002: F, t8116: F, t8177: F, t894: F, t899: F, t913: F, t914: F, t930: F, t953: F) -> F {
    let t26027 = t2797 * t8024;
    let t26039 = t2751 * t2619 * t2753;
    let t26049 = -F::new(0.17386322979577515709e0) * t930 * t914 * t25188 - F::new(0.10747883617784362088e0) * t2704 * t7852 - F::new(0.49917948358154037253e1) * t26010 * t899 + F::new(0.42929192542166705456e-1) * t26014 - F::new(0.21304854723623629356e5) * t26016 * t8116 + F::new(0.71652557451895747254e-1) * t26019 + F::new(0.1343485452223045261e-1) * t26021 + F::new(0.80609127133382715661e-1) * t26023 + F::new(0.2266661366226402048e1) * t8177 * t2806 - F::new(0.41212024840480037237e0) * t26027 + F::new(0.1343485452223045261e0) * t953 * t894 * t7857 * t23825 + F::new(0.11360101276506094136e1) * t913 * t914 * t287 * t24978 * t297 - F::new(0.30972456242994093474e2) * t26039 + F::new(0.69688026546736710315e2) * t2751 * t313 * t25776 * t1 - F::new(0.18583473745796456084e3) * t3907 * t11518 * t25595 * t8002;
    t26049
}
