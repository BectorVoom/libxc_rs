//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1136/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1136<F: Float>(t10817: F, t7736: F, t1853: F, t191: F, t3039: F, t7635: F, t1457: F, t21491: F, t8793: F, t10915: F, t22242: F, t32514: F, t24321: F, t787: F, t9824: F, t10677: F, t10831: F, t10954: F, t1445: F, t1865: F, t28281: F, t28284: F, t32984: F, t32987: F, t32991: F, t32997: F, t33001: F, t33004: F, t4614: F, t5676: F, t813: F) -> (F,) {
    let t33009 = 0.25025342966295298669e1 * t10817 * t7736;
    let t33013 = 0.71500979903700853338e0 * t7635 * t3039 * t191 * t1853;
    let t33018 = 0.50050685932590597338e1 * t8793 * t1457 * t21491;
    let t33021 = 0.42900587942220512002e1 * t22242 * t10915 * t32514;
    let t33023 = t787 * t24321 * t9824;
    let t33024 = 0.14896037479937677779e-1 * t33023;
    let t33025 = -t32984 - t32987 + t32991 - 0.92023022289409799224e1 * t813 * t1445 * t10677 * t1865 + t32997 - t33001 + t33004 - 0.12269736305254639896e2 * t813 * t4614 * t10954 + t28281 - t33009 - t33013 + 0.79445533226334281486e-1 * t5676 * t10831 + t33018 + t33021 + t33024 - t28284;
    (t33025,)
}
