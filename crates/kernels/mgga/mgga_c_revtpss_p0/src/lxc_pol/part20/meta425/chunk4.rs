//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1597/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1597<F: Float>(t43813: F, t43816: F, t43808: F, t43810: F, t43823: F, t43826: F, t43828: F, t43830: F, t43832: F, t43837: F, t43841: F, t43845: F, t43849: F, t43854: F) -> F {
    let t44039 = F::cast_from(0.31003950617283950618e1_f64) * t43813;
    let t44040 = F::cast_from(0.13388493827160493828e1_f64) * t43816;
    let t44051 = -F::new(0.46074375e0) * t43808 + F::new(0.614325e0) * t43810 + t44039 + t44040 - F::cast_from(0.3560484375e1_f64) * t43823 - F::new(0.28483875e1) * t43826 - F::cast_from(0.13145066666666666666e1_f64) * t43828 - F::cast_from(0.23917333333333333334e1_f64) * t43830 + F::cast_from(0.79724444444444444444e0_f64) * t43832 + F::cast_from(0.19931111111111111111e1_f64) * t43837 - F::cast_from(0.79724444444444444444e0_f64) * t43841 + F::new(0.107628e2) * t43845 + F::cast_from(0.23917333333333333333e1_f64) * t43849 - F::cast_from(0.71752000000000000001e1_f64) * t43854;
    t44051
}
