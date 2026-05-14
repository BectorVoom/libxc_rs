//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 599/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk599<F: Float>(t563: F, t8787: F, t1952: F, t2080: F, t520: F, t7773: F, t89: F, t1546: F, t1979: F, t1965: F, t7780: F, t1987: F, t375: F, t1691: F, t7924: F) -> (F, F, F, F, F, F, F) {
    let t8788 = t8787 * t563;
    let t8790 = t1952 * t2080;
    let t8796 = t89 * t7773 * t520;
    let t8799 = t89 * t1546 * t1979;
    let t8802 = t89 * t7780 * t1965;
    let t8805 = t89 * t375 * t1987;
    let t8807 = t1691 * t7924;
    (t8788, t8790, t8796, t8799, t8802, t8805, t8807)
}
