//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 907/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk907(t10861: f64, t10872: f64, t10921: f64, t10923: f64, t10925: f64, t10930: f64, t10932: f64, t10935: f64, t10939: f64, t10943: f64, t10948: f64, t10952: f64, t10961: f64, t10964: f64, t10966: f64, t10969: f64, t10971: f64, t10974: f64, t2754: f64, t2784: f64, t2811: f64, t2815: f64, t4504: f64, t4514: f64, t820: f64, t837: f64) -> f64 {
    let t10976 = 0.16463622957338778996e-1_f64 * t10921 - 0.21951497276451705329e-1_f64 * t10923 + 0.19514881078765566038e-2_f64 * t10925 + 0.32927245914677557992e-1_f64 * t10930 + 0.16463622957338778996e-1_f64 * t10935 + t10939 - 0.19756347548806534796e1_f64 * t4514 * t10932 * t837 + 0.39512695097613069591e1_f64 * t4504 * t2784 * t10943 - t10948 - 0.19756347548806534796e1_f64 * t820 * t2815 * t2754 - 0.39512695097613069591e1_f64 * t820 * t10952 * t10872 + 0.39512695097613069591e1_f64 * t820 * t2811 * t10861 - 0.16463622957338778996e-1_f64 * t10961 - 0.19514881078765566038e-2_f64 * t10964 + 0.21951497276451705329e-1_f64 * t10966 + t10969 - t10971 + 0.29272321618148349057e-1_f64 * t10974;
    t10976
}
