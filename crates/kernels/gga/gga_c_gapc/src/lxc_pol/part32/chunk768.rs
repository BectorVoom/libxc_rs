//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 768/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk768<F: Float>(t185: F, t8998: F, t1723: F, t563: F, t595: F, t1941: F, t8972: F, t8974: F, t8976: F, t8978: F, t8980: F, t8982: F, t8984: F, t8988: F, t8990: F, t8994: F, t8996: F) -> (F, F) {
    let t8999 = t185 * t8998;
    let t9000 = t8999 * t1723;
    let t9002 = t563 * t595;
    let t9003 = t9002 * t1941;
    let t9005 = F::new(0.15176747947735985782e-6) * t8972 - F::new(0.26984257851074582721e-6) * t8974 + F::new(0.21642471925239962898e-3) * t8976 - F::new(0.21642471925239962898e-3) * t8978 - F::new(0.20611878024038059902e-5) * t8980 + F::new(0.36647919126739670507e-5) * t8982 + F::new(0.12380568050579229813e-5) * t8984 + F::new(0.80045999977926802213e-7) * t8988 + F::new(0.27801896084645508334e-2) * t8990 + F::new(0.9275345110817126956e-4) * t8994 + F::new(0.77294542590142724635e-6) * t8996 - F::new(0.1374296967252737644e-5) * t9000 - F::new(0.12357942809624928455e-3) * t9003;
    (t8999, t9005)
}
