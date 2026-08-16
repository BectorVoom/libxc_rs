//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3883/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3883<F: Float>(t2661: F, t3992: F, t48533: F, t6869: F, t14045: F, t22096: F, t21990: F, t5608: F, t9934: F, t1353: F, t13804: F, t13805: F, t1410: F, t21969: F, t22074: F, t22079: F, t3924: F, t3934: F, t3936: F, t4012: F, t47259: F, t47262: F, t5673: F, t74579: F, t74583: F, t74585: F, t74589: F, t828: F) -> F {
    let t74598 = t2661 * t3992 * t48533 * t6869;
    let t74602 = t2661 * t3992 * t14045 * t22096;
    let t74606 = t2661 * t9934 * t5608 * t21990;
    let t74616 = -F::cast_from(0.18071592998981862716e-4_f64) * t47259 + F::cast_from(0.65057734796334705778e-3_f64) * t47262 + F::cast_from(0.11433071498151929859e-3_f64) * t74579 + F::cast_from(0.85748036236139473945e-4_f64) * t74583 - F::cast_from(0.56688979511669985553e-2_f64) * t74585 + F::cast_from(0.28582678745379824648e-3_f64) * t74589 + F::cast_from(0.85748036236139473944e-2_f64) * t1410 * t4012 * t828 * t21969 * t1353 - F::cast_from(0.11433071498151929859e-3_f64) * t74598 - F::cast_from(0.11433071498151929859e-3_f64) * t74602 - F::cast_from(0.57165357490759649296e-4_f64) * t74606 - F::cast_from(0.12862205435420921092e-2_f64) * t13804 * t5673 * t22079 * t13805 + F::cast_from(0.85748036236139473944e-3_f64) * t3934 * t3936 * t22074 * t3924;
    t74616
}
