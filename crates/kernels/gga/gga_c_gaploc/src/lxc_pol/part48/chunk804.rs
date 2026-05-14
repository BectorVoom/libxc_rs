//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 804/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk804<F: Float>(t43930: F, t2365: F, t35500: F, t7390: F, t36798: F, t787: F, t9824: F, t13651: F, t2197: F, t1445: F, t44973: F, t833: F, t45087: F, t13555: F, t2103: F, t4673: F) -> (F, F, F, F, F, F, F) {
    let t45795 = 0.19171462976960374838e0 * t43930;
    let t45797 = t7390 * t2365 * t35500;
    let t45798 = 0.14896037479937677779e-1 * t45797;
    let t45800 = t787 * t36798 * t9824;
    let t45801 = 0.14896037479937677779e-1 * t45800;
    let t45803 = 0.11502877786176224903e2 * t2197 * t13651;
    let t45806 = 0.11502877786176224903e2 * t833 * t1445 * t44973;
    let t45809 = 0.11502877786176224903e2 * t833 * t1445 * t45087;
    let t45812 = 0.47667319935800568892e0 * t2103 * t4673 * t13555;
    (t45795, t45798, t45801, t45803, t45806, t45809, t45812)
}
