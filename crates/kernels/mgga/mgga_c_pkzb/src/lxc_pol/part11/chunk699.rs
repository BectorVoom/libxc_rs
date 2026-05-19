//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 699/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk699<F: Float>(t123: F, t1570: F, t1581: F, t1587: F, t1589: F, t1614: F, t1621: F, t4888: F, t4892: F, t49: F, t4902: F, t4907: F, t4912: F, t4916: F, t4921: F, t4922: F, t4953: F, t4958: F, t4961: F, t4966: F, t4967: F, t4979: F, t4982: F, t4996: F, t5005: F, t5011: F, t520: F, t525: F, t527: F, t535: F) -> F {
    let t5012 = -F::cast_from(0.35089341735807877242e1_f64) * t1614 * t4888 + F::cast_from(0.51947577317044391277e2_f64) * t1621 * t4892 + F::cast_from(0.96491876992155210402e2_f64) * t1587 * t1581 * t1589 * t525 - F::new(6.0) * t1570 * t527 * t1581 + F::cast_from(0.56968947174242584612e-3_f64) * t49 * t4902 * t123 + F::new(6.0) * t1587 * t4907 + F::cast_from(0.10254018858216406658e4_f64) * t4912 * t4916 - F::cast_from(0.10389515463408878255e3_f64) * t4921 * t4922 + F::cast_from(0.5848223622634646207e0_f64) * t535 * t4953 + F::cast_from(0.2069040516770936012e4_f64) * t4958 * t4961 - F::cast_from(0.19298375398431042081e3_f64) * t4966 * t4967 + F::new(1.0) * t520 * t4979 + F::cast_from(0.35089341735807877242e1_f64) * t1621 * t4982 - t4996 - t5005 + t5011;
    t5012
}
