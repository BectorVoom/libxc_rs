//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1220/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1220(t10989: f64, t2099: f64, t5725: f64, t11004: f64, t2945: f64, t10767: f64, t2003: f64, t179: f64, t2068: f64, t299: f64, t10821: f64, t10932: f64, t10934: f64, t10990: f64, t10999: f64, t11005: f64, t18063: f64, t18160: f64, t18210: f64, t21395: f64, t21863: f64, t21870: f64, t25556: f64, t25572: f64, t25576: f64, t25580: f64, t2739: f64, t2887: f64, t29813: f64, t3515: f64, t655: f64, t758: f64, t7607: f64, t771: f64, t7787: f64, t780: f64, t9594: f64) -> f64 {
    let t30049 = t5725 * t2099 * t10989;
    let t30077 = t2945 * t2099 * t11004;
    let t30081 = t2003 * t10767;
    let t30093 = t299 * t179 * t2068 * t10767;
    let t30095 = 0.14291339372689912324e-3_f64 * t25556 + 0.68598428988911579157e-2_f64 * t18160 * t10990 - 0.85748036236139473947e-3_f64 * t30049 - 3.0_f64 / 16.0_f64 * t2887 * t21395 * t10821 * t655 - 0.42874018118069736972e-3_f64 * t299 * t179 * t780 * t29813 - 5.0_f64 / 1296.0_f64 * t18063 + 0.28582678745379824648e-3_f64 * t25572 - 0.13719685797782315831e-1_f64 * t25576 + 0.45732285992607719437e-2_f64 * t25580 + 0.25724410870841842184e-1_f64 * t2945 * t758 * t18210 * t10932 * t655 - 0.1543464652250510531e-1_f64 * t2945 * t758 * t9594 * t2739 + 0.38586616306262763276e-2_f64 * t2945 * t758 * t7787 * t3515 + 0.25724410870841842184e-2_f64 * t30077 - 0.20579528696673473747e-1_f64 * t7607 * t11005 + 0.12862205435420921092e-2_f64 * t2945 * t758 * t30081 * t655 + t21863 - 0.85748036236139473944e-3_f64 * t21870 + 0.27439371595564631663e-1_f64 * t771 * t10934 + 0.22866142996303859718e-2_f64 * t771 * t10999 - 0.28582678745379824648e-3_f64 * t30093;
    t30095
}
