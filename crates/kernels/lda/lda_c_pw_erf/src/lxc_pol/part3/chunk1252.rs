//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1252/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1252<F: Float>(t14903: F, t1131: F, t1187: F, t5931: F, t101: F, t10862: F, t10865: F, t10868: F, t10872: F, t11636: F, t14535: F, t14871: F, t14876: F, t14891: F, t14896: F, t14899: F, t159: F, t1711: F, t1724: F, t1808: F, t1809: F, t1861: F, t1864: F, t1878: F, t1881: F, t2791: F, t281: F, t285: F, t3251: F, t3329: F, t3339: F, t3340: F, t3363: F, t450: F, t456: F, t5618: F, t5667: F, t774: F, t777: F, t8761: F, t9051: F, t9068: F, t9164: F) -> F {
    let t14904 = F::cast_from(5.4655730795145296e-05_f64) * t14903;
    let t14906 = t5931 * t1131 * t1187;
    let t14908 = F::cast_from(6.0_f64) * t1808 * t1809 * t3251 + t101 * (F::cast_from(6.0_f64) * t1711 * t1878 * t1724 + F::cast_from(2.0_f64) * t1711 * t774 * t3363 + F::cast_from(6.0_f64) * t1711 * t5667 * t450 - F::cast_from(18.0_f64) * t3339 * t1864 * t1724 + F::cast_from(24.0_f64) * t9068 * t774 * t3340 - F::cast_from(3.0_f64) * t14535 * t450 - F::cast_from(3.0_f64) * t5618 * t1724 - t1861 * t3363 - F::cast_from(3.0_f64) * t3329 * t1878 - t9051 * t774 + t14871) * t456 + F::cast_from(6.0_f64) * t9164 * t14876 + F::cast_from(3.0_f64) * t1881 * t2791 + t777 * t8761 - F::cast_from(0.0002905674151788692_f64) * t10862 - F::cast_from(0.0017434044910732151_f64) * t10865 - F::cast_from(0.002615106736609823_f64) * t10868 - t10872 - F::cast_from(0.01197423401025461_f64) * t281 * t11636 * t159 * t285 - F::cast_from(0.03592270203076383_f64) * t14891 - t14896 - F::cast_from(0.01197423401025461_f64) * t14899 - t14904 - F::cast_from(5.4655730795145296e-05_f64) * t14906;
    t14908
}
