//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2889/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2889(t2986: f64, t4682: f64, t11465: f64, t1626: f64, t15234: f64, t3014: f64, t11509: f64, t4707: f64, t11399: f64, t11467: f64, t11468: f64, t11501: f64, t11507: f64, t11548: f64, t15258: f64, t15266: f64, t15277: f64, t15280: f64, t15340: f64, t1633: f64, t2938: f64, t2944: f64, t2968: f64, t2988: f64, t2989: f64, t3006: f64, t3012: f64, t41238: f64, t41658: f64, t41759: f64, t41779: f64, t4670: f64, t4708: f64, t4711: f64, t52231: f64, t972: f64) -> f64 {
    let t52440 = t4682 * t2986;
    let t52443 = t1626 * t11465;
    let t52452 = t15234 * t3014;
    let t52459 = t4707 * t11509;
    let t52477 = -t52231 + 3.0_f64 * t11399 * t4670 + 3.0_f64 * t2938 * t15340 - 0.35089341735807877242e1_f64 * t52440 * t2989 - 0.10389515463408878255e3_f64 * t52443 * t11468 + 0.10526802520742363173e2_f64 * t3012 * t4708 * t2988 - 0.12304822629859687989e5_f64 * t41759 * t15266 * t11467 + 0.51947577317044391277e2_f64 * t3012 * t52452 * t972 + 0.51947577317044391277e2_f64 * t3012 * t15258 * t3006 + 0.30762056574649219973e4_f64 * t11507 * t52459 * t2988 + 0.17315859105681463759e2_f64 * t3012 * t4711 * t11501 + 0.91082604192152556044e5_f64 * t41658 * t1633 * t41238 * t11467 + 18.0_f64 * t2968 * t4670 * t2944 - 6.0_f64 * t11548 * t15277 - 0.57895126195293126242e3_f64 * t41779 * t15280;
    t52477
}
