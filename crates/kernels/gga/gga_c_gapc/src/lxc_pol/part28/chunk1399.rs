//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1399/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1399<F: Float>(t34454: F, t34457: F, t34460: F, t34463: F, t34466: F, t34469: F, t34474: F, t34477: F, t34484: F, t34486: F, t34489: F, t34492: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36946 = F::new(0.50680539737635041234e-3) * t34454;
    let t36947 = F::new(0.10298285674687440379e-4) * t34457;
    let t36948 = F::new(0.10298285674687440379e-4) * t34460;
    let t36949 = F::new(0.6070699179094394313e-6) * t34463;
    let t36950 = F::new(0.14068827330203670243e-7) * t34466;
    let t36951 = F::new(0.43284943850479925795e-3) * t34469;
    let t36952 = F::new(0.80966145833333333338e-4) * t34474;
    let t36953 = F::new(0.2845640240200497334e-7) * t34477;
    let t36956 = F::new(0.10762101632577401621e-6) * t34484;
    let t36957 = F::new(0.13259557375557346398e-6) * t34486;
    let t36958 = F::new(0.4637672555408563478e-4) * t34489;
    let t36959 = F::new(0.4637672555408563478e-4) * t34492;
    (t36946, t36947, t36948, t36949, t36950, t36951, t36952, t36953, t36956, t36957, t36958, t36959)
}
