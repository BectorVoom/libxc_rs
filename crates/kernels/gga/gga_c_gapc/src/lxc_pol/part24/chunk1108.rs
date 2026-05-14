//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1108/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1108<F: Float>(t33770: F, t33772: F, t33774: F, t33777: F, t33779: F, t33784: F, t33787: F, t33789: F, t33791: F, t33793: F, t33796: F, t33801: F, t33803: F, t33808: F, t33810: F, t33812: F, t33815: F, t33818: F, t33820: F, t33823: F, t33825: F, t33828: F) -> (F, F) {
    let t37875 = -0.21135226489492151266e-6 * t33770 - 0.8004342540650813035e-7 * t33772 - 0.80189736504692130024e-6 * t33774 + 0.13259130899812740005e-6 * t33777 - 0.11594181388521408695e-4 * t33779 - 0.38672465124453825014e-8 * t33784 + 0.5691280480400994668e-7 * t33787 + 0.6956508833112845217e-4 * t33789 - 0.84410248952307505288e-7 * t33791 + 0.67528199161846004231e-6 * t33793 + 0.42205124476153752644e-7 * t33796;
    let t37888 = 0.80189736504692130024e-6 * t33801 - 0.42205124476153752644e-7 * t33803 - 0.44197102999375800016e-7 * t33808 - 0.11003142262108589692e-5 * t33810 + 0.8096354166666666667e-4 * t33812 + 0.11584123368602295139e-4 * t33815 - 0.10136107947527008247e-2 * t33818 - 0.69504740211613770836e-3 * t33820 + 0.11584123368602295139e-4 * t33823 - 0.10136107947527008247e-2 * t33825 - 0.69504740211613770836e-3 * t33828;
    (t37875, t37888)
}
