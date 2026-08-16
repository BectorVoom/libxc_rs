//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1252/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1252(t14903: f64, t1131: f64, t1187: f64, t5931: f64, t101: f64, t10862: f64, t10865: f64, t10868: f64, t10872: f64, t11636: f64, t14535: f64, t14871: f64, t14876: f64, t14891: f64, t14896: f64, t14899: f64, t159: f64, t1711: f64, t1724: f64, t1808: f64, t1809: f64, t1861: f64, t1864: f64, t1878: f64, t1881: f64, t2791: f64, t281: f64, t285: f64, t3251: f64, t3329: f64, t3339: f64, t3340: f64, t3363: f64, t450: f64, t456: f64, t5618: f64, t5667: f64, t774: f64, t777: f64, t8761: f64, t9051: f64, t9068: f64, t9164: f64) -> f64 {
    let t14904 = 5.4655730795145296e-05_f64 * t14903;
    let t14906 = t5931 * t1131 * t1187;
    let t14908 = 6.0_f64 * t1808 * t1809 * t3251 + t101 * (6.0_f64 * t1711 * t1878 * t1724 + 2.0_f64 * t1711 * t774 * t3363 + 6.0_f64 * t1711 * t5667 * t450 - 18.0_f64 * t3339 * t1864 * t1724 + 24.0_f64 * t9068 * t774 * t3340 - 3.0_f64 * t14535 * t450 - 3.0_f64 * t5618 * t1724 - t1861 * t3363 - 3.0_f64 * t3329 * t1878 - t9051 * t774 + t14871) * t456 + 6.0_f64 * t9164 * t14876 + 3.0_f64 * t1881 * t2791 + t777 * t8761 - 0.0002905674151788692_f64 * t10862 - 0.0017434044910732151_f64 * t10865 - 0.002615106736609823_f64 * t10868 - t10872 - 0.01197423401025461_f64 * t281 * t11636 * t159 * t285 - 0.03592270203076383_f64 * t14891 - t14896 - 0.01197423401025461_f64 * t14899 - t14904 - 5.4655730795145296e-05_f64 * t14906;
    t14908
}
