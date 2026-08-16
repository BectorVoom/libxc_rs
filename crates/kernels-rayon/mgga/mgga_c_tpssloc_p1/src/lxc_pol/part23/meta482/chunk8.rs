//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1459/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1459(t11285: f64, t11350: f64, t11352: f64, t1148: f64, t1156: f64, t1683: f64, t1695: f64, t18840: f64, t18899: f64, t21855: f64, t21887: f64, t21890: f64, t21939: f64, t21942: f64, t3359: f64, t43692: f64, t44155: f64, t44223: f64, t44361: f64, t4797: f64, t4835: f64, t51376: f64, t51427: f64, t51604: f64, t6037: f64, t6053: f64, t6056: f64, t6085: f64, t6088: f64, t63602: f64, t64103: f64, t64292: f64, t71860: f64, t71863: f64, t78114: f64, t78287: f64, t78859: f64) -> f64 {
    let t78914 = -12.0_f64 * t64292 * t6037 - 0.77193501593724168322e3_f64 * t51427 * t21855 + 0.11579025239058625248e4_f64 * t11350 * t78859 * t3359 + 0.23392894490538584828e1_f64 * t71860 * t1695 + 0.35089341735807877242e1_f64 * t18899 * t6085 + 0.10389515463408878255e3_f64 * t63602 * t6088 + 0.23392894490538584828e1_f64 * t4835 * t21939 + 0.4101607543286562663e4_f64 * t51376 * t21942 - 0.12304822629859687989e5_f64 * t44155 * t78287 * t11285 + 0.5848223622634646207e0_f64 * t1148 * t78114 * t1156 + 0.91082604192152556044e5_f64 * t44223 * t78287 * t43692 + 4.0_f64 * t71863 * t1683 + 6.0_f64 * t18840 * t6053 + 0.1929837539843104208e3_f64 * t64103 * t6056 + 4.0_f64 * t4797 * t21887 + 0.82761620670837440481e4_f64 * t51604 * t21890 - 0.24828486201251232145e5_f64 * t44361 * t78859 * t11352;
    t78914
}
