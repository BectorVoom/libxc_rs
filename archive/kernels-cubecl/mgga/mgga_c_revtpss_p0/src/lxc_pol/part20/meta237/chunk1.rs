//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1039/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1039<F: Float>(t10861: F, t10872: F, t10921: F, t10923: F, t10925: F, t10930: F, t10932: F, t10935: F, t10939: F, t10943: F, t10948: F, t10952: F, t10961: F, t10964: F, t10966: F, t10969: F, t10971: F, t10974: F, t2754: F, t2784: F, t2811: F, t2815: F, t4504: F, t4514: F, t820: F, t837: F) -> F {
    let t10976 = F::cast_from(0.16463622957338778996e-1_f64) * t10921 - F::cast_from(0.21951497276451705329e-1_f64) * t10923 + F::cast_from(0.19514881078765566038e-2_f64) * t10925 + F::cast_from(0.32927245914677557992e-1_f64) * t10930 + F::cast_from(0.16463622957338778996e-1_f64) * t10935 + t10939 - F::cast_from(0.19756347548806534796e1_f64) * t4514 * t10932 * t837 + F::cast_from(0.39512695097613069591e1_f64) * t4504 * t2784 * t10943 - t10948 - F::cast_from(0.19756347548806534796e1_f64) * t820 * t2815 * t2754 - F::cast_from(0.39512695097613069591e1_f64) * t820 * t10952 * t10872 + F::cast_from(0.39512695097613069591e1_f64) * t820 * t2811 * t10861 - F::cast_from(0.16463622957338778996e-1_f64) * t10961 - F::cast_from(0.19514881078765566038e-2_f64) * t10964 + F::cast_from(0.21951497276451705329e-1_f64) * t10966 + t10969 - t10971 + F::cast_from(0.29272321618148349057e-1_f64) * t10974;
    t10976
}
