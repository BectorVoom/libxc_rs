//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 770/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk770<F: Float>(t1445: F, t44973: F, t833: F, t45087: F, t13555: F, t2103: F, t4673: F, t13644: F, t5782: F, t2033: F, t2365: F, t35451: F, t11784: F, t2679: F, t9800: F, t2617: F, t3626: F, t7810: F) -> (F, F, F, F, F, F, F) {
    let t45806 = 0.11502877786176224903e2 * t833 * t1445 * t44973;
    let t45809 = 0.11502877786176224903e2 * t833 * t1445 * t45087;
    let t45812 = 0.47667319935800568892e0 * t2103 * t4673 * t13555;
    let t45817 = 0.62115540045351614476e2 * t5782 * t13644;
    let t45819 = t2033 * t2365 * t35451;
    let t45820 = 0.44688112439813033337e-1 * t45819;
    let t45822 = t9800 * t11784 * t2679;
    let t45823 = 0.9585731488480187419e0 * t45822;
    let t45826 = t7810 * t3626 * t2617;
    (t45806, t45809, t45812, t45817, t45820, t45823, t45826)
}
