//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 717/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk717(t12459: f64, t955: f64, t179: f64, t918: f64, t2861: f64, t1016: f64, t4: f64, t15: f64, t963: f64, t12630: f64, t12723: f64, t12730: f64, t12735: f64, t12741: f64, t12747: f64, t180: f64, t182: f64, t183: f64, t2925: f64, t3144: f64, t3155: f64, t3156: f64, t3162: f64, t3166: f64, t456: f64, t60: f64, t852: f64, t983: f64, t990: f64, t991: f64, t995: f64) -> f64 {
    let t12757 = t12459 * t955;
    let t12760 = t179 * t918;
    let t12761 = t12760 * t2861;
    let t12762 = t1016 * t4;
    let t12765 = t963 * t15;
    let t12769 = -4.0_f64 * t60 * t12630 - 12.0_f64 * t852 * t2925 - 0.19711288999999999999e-2_f64 * t180 * t182 * t12723 - 0.59133866999999999997e-2_f64 * t990 * t991 * t3162 - 0.11826773399999999999e-1_f64 * t180 * t182 * t12730 + 0.11826773399999999999e-1_f64 * t456 * t12735 + 0.78845155999999999997e-2_f64 * t180 * t983 * t3162 - 0.39422577999999999998e-2_f64 * t12741 * t3156 + 0.29566933499999999998e-2_f64 * t990 * t991 * t3166 + 0.58403819259259259257e-3_f64 * t180 * t12747 * t183 + 0.13140859333333333333e-2_f64 * t180 * t3144 * t995 - 0.39422577999999999999e-2_f64 * t180 * t983 * t3166 - 0.59133866999999999997e-2_f64 * t3155 * t12757 + 0.13140859333333333333e-2_f64 * t12761 * t12762 + 0.21901432222222222225e-3_f64 * t990 * t12765 * t183;
    t12769
}
