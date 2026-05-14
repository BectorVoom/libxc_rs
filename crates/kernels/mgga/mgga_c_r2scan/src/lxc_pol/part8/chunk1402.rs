//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1402/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1402<F: Float>(t133: F, t1604: F, t22980: F, t9937: F, t2609: F, t31060: F, t2562: F, t3216: F, t2148: F, t7628: F, t20646: F, t494: F, t10024: F, t20642: F, t20769: F, t2124: F, t24695: F, t2557: F, t2569: F, t2591: F, t2598: F, t2634: F, t27870: F, t29851: F, t29861: F, t29866: F, t29892: F, t32896: F, t360: F, t7433: F, t8783: F, t9955: F) -> (F, F) {
    let t33988 = t1604 * t22980 * t133 * t9937;
    let t33991 = t31060 * t2609;
    let t33994 = t2562 * t3216;
    let t33996 = t7628 * t2148 * t33994;
    let t33999 = t20646 * t494;
    let t34019 = -0.69345773920434148506e0 * t29851 + 0.65854491829355115988e-1 * t33988 + 0.20803732176130244552e1 * t29861 - 0.17465477326173296717e-1 * t33991 + t20769 - 0.34930954652346593433e-1 * t29866 + 0.34930954652346593433e-1 * t33996 + 0.65854491829355115988e0 * t2557 * t2124 * t20642 * t9955 * t33999 + 0.86682217400542685632e-1 * t2598 * t360 * t32896 * t2591 + 0.31205598264195366828e1 * t24695 * t360 * t8783 * t2634 + 0.26004665220162805689e0 * t2598 * t360 * t7433 * t10024 + 0.39006997830244208535e0 * t27870 * t2569 - 0.69345773920434148506e0 * t29892;
    (t33999, t34019)
}
