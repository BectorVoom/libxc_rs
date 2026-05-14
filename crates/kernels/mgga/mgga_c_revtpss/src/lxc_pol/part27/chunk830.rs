//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 830/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk830<F: Float>(t10972: F, t686: F, t874: F, t10861: F, t10872: F, t10921: F, t10923: F, t10925: F, t10930: F, t10932: F, t10935: F, t10939: F, t10943: F, t10948: F, t10952: F, t10961: F, t10964: F, t10966: F, t10969: F, t10971: F, t2754: F, t2784: F, t2811: F, t2815: F, t4504: F, t4514: F, t820: F, t837: F) -> (F,) {
    let t10974 = t874 * t10972 * t686;
    let t10976 = 0.16463622957338778996e-1 * t10921 - 0.21951497276451705329e-1 * t10923 + 0.19514881078765566038e-2 * t10925 + 0.32927245914677557992e-1 * t10930 + 0.16463622957338778996e-1 * t10935 + t10939 - 0.19756347548806534796e1 * t4514 * t10932 * t837 + 0.39512695097613069591e1 * t4504 * t2784 * t10943 - t10948 - 0.19756347548806534796e1 * t820 * t2815 * t2754 - 0.39512695097613069591e1 * t820 * t10952 * t10872 + 0.39512695097613069591e1 * t820 * t2811 * t10861 - 0.16463622957338778996e-1 * t10961 - 0.19514881078765566038e-2 * t10964 + 0.21951497276451705329e-1 * t10966 + t10969 - t10971 + 0.29272321618148349057e-1 * t10974;
    (t10976,)
}
