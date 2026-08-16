//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1197/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1197(t15426: f64, t68: f64, t484: f64, t11836: f64, t11839: f64, t11842: f64, t1227: f64, t15727: f64, t15731: f64, t15735: f64, t15737: f64, t15740: f64, t15745: f64, t15750: f64, t15754: f64, t15761: f64, t3490: f64, t3511: f64, t3577: f64, t3580: f64, t3587: f64, t488: f64, t5024: f64, t5030: f64) -> f64 {
    let t15764 = t15426 * t68;
    let t15765 = t15764 * t484;
    let t15768 = t15727 / 162.0_f64 - t15731 / 13824.0_f64 + t15735 / 20736.0_f64 + t15737 * t3511 / 1536.0_f64 - t15740 * t3580 / 2304.0_f64 + t15745 + t11836 / 648.0_f64 - t11839 / 864.0_f64 - t11842 / 432.0_f64 + 5.0_f64 / 6912.0_f64 * t3577 * t15750 + t15754 / 1296.0_f64 - 5.0_f64 / 2592.0_f64 * t5024 * t3587 - t3490 * t5030 / 2304.0_f64 - t1227 * t15761 / 4608.0_f64 + t15765 * t488 / 3072.0_f64;
    t15768
}
