//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2889/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2889<F: Float>(t2986: F, t4682: F, t11465: F, t1626: F, t15234: F, t3014: F, t11509: F, t4707: F, t11399: F, t11467: F, t11468: F, t11501: F, t11507: F, t11548: F, t15258: F, t15266: F, t15277: F, t15280: F, t15340: F, t1633: F, t2938: F, t2944: F, t2968: F, t2988: F, t2989: F, t3006: F, t3012: F, t41238: F, t41658: F, t41759: F, t41779: F, t4670: F, t4708: F, t4711: F, t52231: F, t972: F) -> F {
    let t52440 = t4682 * t2986;
    let t52443 = t1626 * t11465;
    let t52452 = t15234 * t3014;
    let t52459 = t4707 * t11509;
    let t52477 = -t52231 + F::new(3.0) * t11399 * t4670 + F::new(3.0) * t2938 * t15340 - F::cast_from(0.35089341735807877242e1_f64) * t52440 * t2989 - F::cast_from(0.10389515463408878255e3_f64) * t52443 * t11468 + F::cast_from(0.10526802520742363173e2_f64) * t3012 * t4708 * t2988 - F::cast_from(0.12304822629859687989e5_f64) * t41759 * t15266 * t11467 + F::cast_from(0.51947577317044391277e2_f64) * t3012 * t52452 * t972 + F::cast_from(0.51947577317044391277e2_f64) * t3012 * t15258 * t3006 + F::cast_from(0.30762056574649219973e4_f64) * t11507 * t52459 * t2988 + F::cast_from(0.17315859105681463759e2_f64) * t3012 * t4711 * t11501 + F::cast_from(0.91082604192152556044e5_f64) * t41658 * t1633 * t41238 * t11467 + F::new(18.0) * t2968 * t4670 * t2944 - F::new(6.0) * t11548 * t15277 - F::cast_from(0.57895126195293126242e3_f64) * t41779 * t15280;
    t52477
}
